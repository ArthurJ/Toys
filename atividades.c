/*
Nome: Arthur Evangelista Santos de Sao Julião
Matrícula: 19100190
*/

int ativ_17(){
    /*Potenciação*/
    int n, i;
    float x, pot = 1.0;

    printf("\nDigite a base e o expoente: ");
    scanf("%f %d", &x, &n);

    for (i = n; i >0; i--)
        pot *= x;
    printf("\n%.2f ** %d = %.2f", x, n, pot);
    return 0;
}

int ativ_19(){
    /*Quadrados por soma dos n primeiros números ímpares */
    int x, i, cont=1, res=0;
    printf("\nDigite a base: ");
    scanf("%u", &x);

    for(i=0;i<x;i++){
        res+=cont;
        cont+=2;
    }

    printf("%d**2=%d",x,res);

    return 0;
}

int ativ_20(){
    /*N-ésimo número triangular*/
    int x, i, next=1, n_tri=0;
    
    printf("Entre com o número N: ");
    scanf("%u", &x);
    
    for(i=0;i<x;i++){
        n_tri+=next;
        next++;
    }

    printf("O N-ésimo número triangular é: %d", n_tri);
    return 0;
}



int ativ_18(){
    /*Data válida?*/
    int d,m,a;
    int valido=1;
    printf("Entre com dia, mes e ano (separados por espaço): ");
    scanf("%u %u %u", &d, &m, &a);

    if(m>=13){
        valido*=0;
    }

    if (m==2){
        if (a%400==0 || (a%4==0 && a%100!=0)){
            if (d>=30){
                valido*=0;
            }
        }else{
            if (d>=29){
                //printf("\n%.4d%.2d%.2d\n%d\n",a,m,d,valido);
                valido*=0;
            } 
        }
    }else{
        switch(m){
            case 1:;
            case 3:;
            case 5:;
            case 7:;
            case 8:;
            case 10:;
            case 12: if(d>=32) valido*=0; break;
            case 4:;
            case 6:;
            case 9:;
            case 11: if(d>=31) valido*=0; break;
        }
    }

    if (valido==1){
        printf("Data válida.\n");
    }else{
        printf("Data inválida.\n");
        ativ_18();
    }
    return 0;
}
